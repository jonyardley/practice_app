/// A description
import SharedTypes
import SwiftUI

struct ContentView: View {
    @ObservedObject var core: Core

    init(core: Core) {
        self.core = core
    }

    var body: some View {
        TabView {
            ExercisesView(core: core)
                .tabItem {
                    Label("Exercises", systemImage: "pianokeys")
                }
            SessionsView(core: core)
                .tabItem {
                    Label("Sessions", systemImage: "clock")
                }
            SettingsView()
                .tabItem {
                    Label("Settings", systemImage: "gear")
                }
        }
    }

    struct ExercisesView: View {
        @ObservedObject var core: Core
        @State private var newExersizeName: String = ""

        var body: some View {
            VStack(alignment: .leading) {
                Text("Exercises")
                    .font(.largeTitle)
                    .padding(.bottom, 10)
                    .padding(.leading)

                HStack {
                    TextField("New Exersize", text: $newExersizeName)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .padding(.leading)
                    Button(action: {
                        core.update(.addExersize($newExersizeName.wrappedValue))
                        newExersizeName = ""
                    }) {
                        Text("Add")
                            .fontWeight(.bold)
                            .font(.body)
                            .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                            .background(Color.blue)
                            .cornerRadius(10)
                            .foregroundColor(.white)
                            .padding(.trailing)
                    }
                }
                .padding(.bottom, 10)

                List {
                    Section(header: Text("Your Exersizes")) {
                        ForEach(core.view.exersizes, id: \.self.id) { exersize in
                            HStack {
                                Text(exersize.name)
                                //Spacer()
                                //CheckBoxView()
                            }
                        }
                    }
                }

            }
        }
    }

    struct CheckBoxView: View {
        @State private var isChecked: Bool = false

        var body: some View {
            Button(action: {
                isChecked.toggle()
            }) {
                Image(systemName: isChecked ? "checkmark.square" : "square")
                    .foregroundColor(isChecked ? .blue : .gray)
            }
        }
    }

    struct SessionsView: View {
        @ObservedObject var core: Core
        @State private var newSessionName: String = ""

        var body: some View {
            VStack(alignment: .leading) {
                Text("Sessions")
                    .font(.largeTitle)
                    .padding(.bottom, 10)
                    .padding(.leading)

                HStack {
                    TextField("New Session", text: $newSessionName)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .padding(.leading)
                    Button(action: {
                        core.update(.addSession($newSessionName.wrappedValue))
                        newSessionName = ""
                    }) {
                        Text("Add")
                            .fontWeight(.bold)
                            .font(.body)
                            .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                            .background(Color.blue)
                            .cornerRadius(10)
                            .foregroundColor(.white)
                            .padding(.trailing)
                    }
                }
                .padding(.bottom, 10)

                List {
                    Section(header: Text("Your Sessions")) {
                        ForEach(core.view.sessions, id: \.self.id) { session in
                            HStack {
                                Text(session.name)
                                //Spacer()
                                //CheckBoxView()
                            }
                        }
                    }
                }
            }
        }
    }

    struct SettingsView: View {
        var body: some View {
            Text("Settings")
        }
    }
}

struct ActionButton: View {
    var label: String
    var color: Color
    var action: () -> Void

    init(label: String, color: Color, action: @escaping () -> Void) {
        self.label = label
        self.color = color
        self.action = action
    }

    var body: some View {
        Button(action: action) {
            Text(label)
                .fontWeight(.bold)
                .font(.body)
                .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                .background(color)
                .cornerRadius(10)
                .foregroundColor(.white)
                .padding()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(core: Core())
    }
}
