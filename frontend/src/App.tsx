import { useState } from "react";
import styles from "./App.module.css";

import Navbar from "./components/Navbar/Navbar";
import Auth from "./components/Auth/Auth";

export default function App() {
    const [section, setSection] = useState("YOU");
    const items = ["FIND", "NOTE", "DONE", "YOU"];

    return <main className={styles.parent}>
        <Navbar items={items} setSection={setSection} />
        <section className={styles.main}>
            {select_component(section)}
        </section>
    </main>
}

function select_component(section: string): JSX.Element {
    switch (section) {
        case "FIND": return <></>;
        case "NOTE": return <></>;
        case "DONE": return <></>;
        case "YOU": return <Auth />;
        default: return <></>;
    }
}
