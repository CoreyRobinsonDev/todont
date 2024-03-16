import { useState } from "react";
import styles from "./Auth.module.css";
import { User } from "../../util/types";
import { BASE_URL, getCookie } from "../../util/api";

export default function Auth() {
    const [user, setUser] = useState<User>({
        id: 0,
        email: "",
        password: "",
        created_at: "",
        updated_at: ""
    });
    const [password, setPassword] = useState("");

    const create_account = (e:React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        fetch(BASE_URL + "/users", {
            method: "POST",
            headers: {
                "Content-type": "application/json"
            },
            body: JSON.stringify({
                email: user.email,
                password: user.password,
                confirm_password: password
            })
        }).then(res => {
                console.log(res);
                return res.json();
            })
        .then(data => console.log(data))
    }

    const log_in = (e:React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        fetch(BASE_URL + "/users/log_in", {
            method: "POST",
            headers: {
                "Content-type": "application/json"
            },
            body: JSON.stringify({
                email: user.email,
                password: user.password,
            })
        }).then(res => {
                console.log(res);
                return res.json();
            })
        .then(data => console.log(data))

    }
    console.log(getCookie("auth-token"));

    return <section className={styles.main}>
        <span 
            style={{letterSpacing: `${(70 - (user.email?.length * 2.3)) < -3
                ? 0
                : (70 - (user.email?.length * 2.3))}px`}}
            className={styles.email}>
            {user?.email || "guest"}</span>
        <form 
            onSubmit={(e) => create_account(e)}
            className={styles.create_account_container}>
            <label className={styles.input} aria-label="email">
                <input 
                    type="email" 
                    value={user?.email} 
                    onChange={(e) => setUser((u) => ({ ...u, email: e.target.value }))}
                    placeholder="" />
                <span className={styles.placeholder}>email</span>
            </label>
            <label className={styles.input} aria-label="password">
                <input 
                    type="password" 
                    onChange={(e) => setUser((u) => ({ ...u, password: e.target.value }))}
                    value={user?.password} placeholder="" />
                <span className={styles.placeholder}>password</span>
            </label>
            <label className={styles.input} aria-label="confirm password">
                <input 
                    type="password" 
                    onChange={(e) => setPassword(e.target.value)}
                    value={password} placeholder="" />
                <span className={styles.placeholder}>confirm password</span>
            </label>
            <button className={styles.button} type="submit">create account</button>
        </form>
        <form 
            onSubmit={(e) => log_in(e)}
            className={styles.login_container}>
            <label className={styles.input} aria-label="email">
                <input 
                    type="email" 
                    value={user?.email} 
                    onChange={(e) => setUser((u) => ({ ...u, email: e.target.value }))}
                    placeholder="" />
                <span className={styles.placeholder}>email</span>
            </label>
            <label className={styles.input} aria-label="password">
                <input 
                    type="password" 
                    onChange={(e) => setUser((u) => ({ ...u, password: e.target.value }))}
                    value={user?.password} 
                    placeholder="" />
                <span className={styles.placeholder}>password</span>
            </label>
            <button className={styles.button} type="submit">login</button>
        </form>
    </section>
}



