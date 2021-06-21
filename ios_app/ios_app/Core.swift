import Foundation

protocol Core {
    /// Performs initialization in Rust (e.g. creating the dependency tree)
    func bootstrap() -> Result<(), CoreError>

    /// Retrieves infos for an Address
    func getInfos(address: String) -> Result<AccountViewData, CoreError>
}

struct CoreImpl: Core {

    func bootstrap() -> Result<(), CoreError> {
        let res = ffi_bootstrap()
        print("bootstrap res: \(res)")
        return .success(())
    }

    // For brevity, this implementation uses force unwrap
    func getInfos(address: String) -> Result<AccountViewData, CoreError> {
        let ffi_infos = ffi_get_infos(address);
        let status = ffi_infos.status
        let error = ffi_infos.error!.toString()
        switch status {
        case 1:
            let viewData = ffi_infos.data.toViewData()
            print("View data: \(viewData)")
            return .success(viewData)
        case 2:
            print("FFI error: \(error)")
            return .failure(.unknown)
        case _:
            print("Unknown status: \(ffi_infos.status)")
            return .failure(.unknown)
        }
    }
}

enum CoreError: Error {
    case unknown
}

// Convenience
extension FFIAccountViewData {
    func toViewData() -> AccountViewData {
        AccountViewData(
            address: address!.toString(),
            status: status!.toString(),
            holdings: holdings!.toString(),
            rewards: rewards!.toString(),
            pending_rewards: pending_rewards!.toString()
        )
    }
}


// Convenience
extension Unmanaged where Instance == CFString {
    func toString() -> String {
        let resultValue: CFString = takeRetainedValue()
        return resultValue as String
    }
}
