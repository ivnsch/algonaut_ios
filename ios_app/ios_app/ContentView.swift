import SwiftUI

struct ContentView: View {
    @ObservedObject private var viewModel: MyViewModel

    init(viewModel: MyViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        VStack {
            HStack {
                TextField("", text: $viewModel.addressInput)
                    .padding(.leading, 10)
                    .padding(.trailing, 10)
                    .padding(.top, 4)
                    .padding(.bottom, 4)
                    .overlay(RoundedRectangle(cornerRadius: 14).stroke(Color.gray, lineWidth: 1))

                Button(action: {
                    viewModel.submitAddress()
                }) { Text("Get") }

            }
            List(viewModel.entries, id: \.label) { entry in
                VStack(alignment: .leading) {
                    Text(entry.label).font(.system(size: 10))
                    Text(entry.value).bold()
                }
            }
        }.padding(.leading, 16).padding(.trailing, 16).padding(.top, 20)
    }
}
