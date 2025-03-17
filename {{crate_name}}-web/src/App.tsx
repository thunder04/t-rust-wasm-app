import { memory } from "@{{crate_name}}/native-lib";
import "./App.scss";

export default function App() {
    return <div>Memory length: {memory.length}</div>;
}
