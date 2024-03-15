import { useState } from "react";
import styles from "./Auth.module.css";

export default function Auth() {
    const [email, _setEmail] = useState("Guest");

    return <section className={styles.main}>
        <p>{email}</p>

    </section>
}

