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
            ExercisesView()
                .tabItem {
                    Label("Exercises", systemImage: "pianokeys")
                }
            SessionsView()
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
        var body: some View {
            Text("Exercises")
        }
    }

    struct SessionsView: View {
        var body: some View {
            Text("Sessions")
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
