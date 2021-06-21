import SwiftUI

@main
struct MyApp: App {
    private let core = initCore()

    var body: some Scene {
        WindowGroup {
            ContentView(viewModel: MyViewModel(core: core))
        }
    }

    // In a real app you can instantiate Core with a dependency injection framework
    static func initCore() -> Core {
        let core = CoreImpl()
        let res = core.bootstrap()
        // Handle error in a real project
        print("Bootstrap res: \(res)")
        return core
    }
}
