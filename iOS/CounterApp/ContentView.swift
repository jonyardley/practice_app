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
        let majorScales: [String] = [
            "C Major", "G Major", "D Major", "A Major", "E Major", "B Major",
            "F# Major / Gb Major", "C# Major / Db Major", "G# Major / Ab Major",
            "D# Major / Eb Major", "A# Major / Bb Major", "F Major",
        ]

        let minorScales: [String] = [
            "A Minor", "E Minor", "B Minor", "F# Minor / Gb Minor", "C# Minor / Db Minor",
            "G# Minor / Ab Minor",
            "D# Minor / Eb Minor", "A# Minor / Bb Minor", "F Minor", "C Minor", "G Minor",
            "D Minor",
        ]

        var body: some View {
            VStack(alignment: .leading) {
                Text("Exercises")
                    .font(.largeTitle)
                    .padding(.bottom, 10)
                    .padding(.leading)

                List {
                    Section(header: Text("Major Scales")) {
                        ForEach(majorScales, id: \.self) { scale in
                            HStack {
                                Text(scale)
                                Spacer()
                                CheckBoxView()
                            }
                        }
                    }

                    Section(header: Text("Minor Scales")) {
                        ForEach(minorScales, id: \.self) { scale in
                            HStack {
                                Text(scale)
                                Spacer()
                                CheckBoxView()
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
