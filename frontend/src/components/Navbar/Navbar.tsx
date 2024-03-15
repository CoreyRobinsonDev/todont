import { useState } from "react";

import styles from "./Navbar.module.css";
import Title from "../Title/Title";
import Auth from "../Auth/Auth";

export default function Navbar() {
    const [section, setSection] = useState("NOTE");
    const items = ["NOTE", "DONE", "YOU"];

    return <nav className={styles.main}>
        <Title name={section} />
        <ul className={styles.list}>
            {items.map((item, i) =>
                <li tabIndex={0} 
                    key={`item-${i}`}
                    onFocus={(e) => {
                        [...e.currentTarget.parentElement!.children].forEach((child) => {
                            child.classList.remove(styles.focus);
                        })
                        e.currentTarget.classList.add(styles.focus);
                    }}
                    className={styles.item}
                    onClick={(e) => setSection(e.currentTarget.innerText)}
                >{item}</li>
            )}
        </ul>
        <Auth/>
    </nav>
}
