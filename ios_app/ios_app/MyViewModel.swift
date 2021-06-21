import Foundation
import Combine

struct EntryViewData {
    let label: String
    let value: String
}

class MyViewModel: ObservableObject {
    private let core: Core
    @Published var entries: [EntryViewData] = []
    @Published var addressInput: String = ""

    private let refreshTrigger = PassthroughSubject<(), Never>()

    private var cancellables: Set<AnyCancellable> = []

    init(core: Core) {
        self.core = core
    }

    func submitAddress() {
        updateInfos()
    }

    private func updateInfos() {
        switch core.getInfos(address: addressInput) {
        case .success(let viewData): updateView(viewData: viewData)
        case .failure(let e): print("Error in view model: \(e)")
        }
    }

    private func updateView(viewData: AccountViewData) {
        entries = [
            EntryViewData(label: "Status", value: viewData.status),
            EntryViewData(label: "Holdings", value: viewData.holdings),
            EntryViewData(label: "Rewards", value: viewData.rewards),
            EntryViewData(label: "Pending rewards", value: viewData.pending_rewards)
        ]
    }
}

struct AccountViewData: Decodable {
    let address: String
    let status: String
    let holdings: String
    let rewards: String
    let pending_rewards: String
}
