/// A description
import SharedTypes
import SwiftUI

struct ContentView: View {
    @ObservedObject var core: Core
    var devInit: Bool = false

    init(core: Core) {
        self.core = core
        self.core.update(.devInit)
    }

    var body: some View {
        TabView {
            SessionsView(core: core)
                .tabItem {
                    Label("Sessions", systemImage: "clock")
                }
            ExercisesView(core: core)
                .tabItem {
                    Label("Exercises", systemImage: "pianokeys")
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
            NavigationView {
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
                                NavigationLink(destination: SessionDetailView(session: session)) {
                                    HStack {
                                        Text(session.name)
                                    }
                                }
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

struct SessionDetailView: View {
    var session: Session

    var body: some View {
        VStack(alignment: .leading) {
            Text(session.name)
                .font(.largeTitle)
                .padding(.bottom, 10)
                .padding(.leading)

            Button(action: {
                //core.update(.addSession($newSessionName.wrappedValue))
                //newSessionName = ""
            }) {
                Text("Add/Edit Exersizes")
                    .fontWeight(.bold)
                    .font(.body)
                    .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                    .background(Color.blue)
                    .cornerRadius(10)
                    .foregroundColor(.white)
                    .padding(.trailing)
            }
            Spacer()
        }
        .navigationBarTitle(Text("Session Details"), displayMode: .inline)
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
