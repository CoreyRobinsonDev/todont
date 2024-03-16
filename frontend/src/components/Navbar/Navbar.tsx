import { useState } from "react";

import styles from "./Navbar.module.css";
import Title from "../Title/Title";

type Props = {
    items: string[]
    setSection: React.Dispatch<React.SetStateAction<string>>
}
export default function Navbar({ items, setSection }: Props) {
    const [section, setLocalSection] = useState("YOU");

    return <nav className={styles.main}>
        <Title name={section} />
        <ul className={styles.list}>
            {items.map((item, i) =>
                <li tabIndex={0} 
                    key={`item-${i}`}
                    className={`${styles.item} ${
                        section === item ? styles.focus : ""
                    }`}
                    onClick={(e) => {
                        setSection(e.currentTarget.innerText);
                        setLocalSection(e.currentTarget.innerText);
                    }}
                >{item}</li>
            )}
        </ul>
    </nav>
}
