import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import log from "./logger";

interface User {
  id: number;
  name: string;
  email: string;
}

// Define a type for the log level
type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

const logToBackend = async (
  level: LogLevel,
  message: string
): Promise<void> => {
  try {
    await invoke("log_message", { level, message });
  } catch (error) {
    log.error("Failed to send log to backend:", error);
  }
};

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");
  const [users, setUsers] = useState<User[]>([]);

  async function get() {
    try {
      // Example of logging an info message
      // log.info("This is an info message");
      // logToBackend("info", "This is an info message from ReactJS");

      const users = await invoke<User[]>("fetch_users");
      logToBackend("info", `Return users: ${JSON.stringify(users)}`);
      setUsers(users);
    } catch (error) {
      // console.error("Failed to fetch users:", error); // Example of logging an info message
      log.error("Failed to fetch users:", error);
      logToBackend("error", `Failed to fetch users. ${JSON.stringify(users)}`);
    }
  }
  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      {/* <p>Click on the Tauri, Vite, and React logos to learn more.</p> */}

      {/* <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p> */}

      <div>
        <h1>User List</h1>
        <button
          onClick={(e) => {
            e.preventDefault();
            get();
          }}
        >
          Click me!
        </button>
        <ul>
          {users.map((user) => (
            <li key={user.id}>
              {user.name} ({user.email})
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;
