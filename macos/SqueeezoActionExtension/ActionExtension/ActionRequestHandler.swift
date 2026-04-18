import Foundation
import UniformTypeIdentifiers

private enum ExtensionError: LocalizedError {
    case invalidSelection
    case unsupportedType
    case helperMissing
    case malformedResponse
    case compressionFailed(String)

    var errorDescription: String? {
        switch self {
        case .invalidSelection:
            return "Squeeezo Quick Action expects exactly one selected file."
        case .unsupportedType:
            return "Squeeezo Quick Action only supports PDF files."
        case .helperMissing:
            return "Bundled compression helper is missing from the extension."
        case .malformedResponse:
            return "The compression helper returned an unreadable response."
        case .compressionFailed(let message):
            return message
        }
    }
}

private struct CompressionCLIResult: Decodable {
    let status: String
    let outputPath: String?
    let error: CompressionCLIError?
}

private struct CompressionCLIError: Decodable {
    let code: String
    let message: String
    let details: String?
}

final class ActionRequestHandler: NSObject, NSExtensionRequestHandling {
    private let defaultSuffix = ".compressed"
    private let sharedDefaultsSuite = "group.com.squeeezo.shared"

    func beginRequest(with context: NSExtensionContext) {
        resolveSinglePDFURL(from: context.inputItems) { [weak self] result in
            guard let self else {
                context.cancelRequest(withError: ExtensionError.invalidSelection as NSError)
                return
            }

            switch result {
            case .success(let inputURL):
                DispatchQueue.global(qos: .userInitiated).async {
                    do {
                        let compressionResult = try self.runCompression(for: inputURL)
                        if compressionResult.status == "failed" {
                            let detail = compressionResult.error.map { "\($0.code): \($0.message)" }
                                ?? "Compression failed."
                            throw ExtensionError.compressionFailed(detail)
                        }

                        context.completeRequest(returningItems: [], completionHandler: nil)
                    } catch {
                        context.cancelRequest(withError: error as NSError)
                    }
                }
            case .failure(let error):
                context.cancelRequest(withError: error as NSError)
            }
        }
    }

    private func resolveSinglePDFURL(
        from inputItems: [Any],
        completion: @escaping (Result<URL, Error>) -> Void
    ) {
        let attachments = inputItems
            .compactMap { $0 as? NSExtensionItem }
            .flatMap { $0.attachments ?? [] }

        guard attachments.count == 1 else {
            completion(.failure(ExtensionError.invalidSelection))
            return
        }

        let provider = attachments[0]
        provider.loadItem(
            forTypeIdentifier: UTType.fileURL.identifier,
            options: nil
        ) { item, error in
            if let error {
                completion(.failure(error))
                return
            }

            if let url = item as? URL {
                completion(self.validate(url: url))
                return
            }

            if let data = item as? Data,
               let url = NSURL(
                absoluteURLWithDataRepresentation: data,
                relativeTo: nil
               ) as URL?
            {
                completion(self.validate(url: url))
                return
            }

            completion(.failure(ExtensionError.unsupportedType))
        }
    }

    private func validate(url: URL) -> Result<URL, Error> {
        guard isPDF(url: url) else {
            return .failure(ExtensionError.unsupportedType)
        }

        return .success(url)
    }

    private func isPDF(url: URL) -> Bool {
        if let contentType = try? url.resourceValues(forKeys: [.contentTypeKey]).contentType {
            return contentType.conforms(to: .pdf)
        }

        return url.pathExtension.caseInsensitiveCompare("pdf") == .orderedSame
    }

    private func runCompression(for inputURL: URL) throws -> CompressionCLIResult {
        guard let helperURL = Bundle.main.resourceURL?.appendingPathComponent("compression-cli"),
              FileManager.default.isExecutableFile(atPath: helperURL.path)
        else {
            throw ExtensionError.helperMissing
        }

        let process = Process()
        process.executableURL = helperURL
        process.arguments = [
            "--input", inputURL.path,
            "--suffix", outputSuffix(),
            "--source", "finder-action",
            "--json",
        ]

        let stdoutPipe = Pipe()
        let stderrPipe = Pipe()
        process.standardOutput = stdoutPipe
        process.standardError = stderrPipe

        try process.run()
        process.waitUntilExit()

        let stdout = stdoutPipe.fileHandleForReading.readDataToEndOfFile()
        let stderr = stderrPipe.fileHandleForReading.readDataToEndOfFile()

        guard !stdout.isEmpty else {
            let stderrText = String(data: stderr, encoding: .utf8) ?? "Compression helper failed."
            throw ExtensionError.compressionFailed(stderrText.trimmingCharacters(in: .whitespacesAndNewlines))
        }

        let decoder = JSONDecoder()
        guard let result = try? decoder.decode(CompressionCLIResult.self, from: stdout) else {
            throw ExtensionError.malformedResponse
        }

        if process.terminationStatus != 0,
           let error = result.error
        {
            let detailsSuffix = error.details.map { " \($0)" } ?? ""
            throw ExtensionError.compressionFailed("\(error.code): \(error.message)\(detailsSuffix)")
        }

        return result
    }

    private func outputSuffix() -> String {
        UserDefaults(suiteName: sharedDefaultsSuite)?.string(forKey: "outputSuffix")
            ?? defaultSuffix
    }
}
