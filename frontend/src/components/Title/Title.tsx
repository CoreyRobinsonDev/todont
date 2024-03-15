import styles from "./Title.module.css";

type Props = {
    name: String
}

export default function Title({ name }: Props) {
    return <h1 className={styles.main}>
        <span className={styles.letter}>{name[0]}</span>
        <span className={styles.letter}>{name[1]}</span>
        <span className={styles.letter}>{name[2]}</span>
        {
            name[3] 
            ? <span className={styles.letter}>{name[3]}</span>
            : <span className={styles.blank}></span>
        }
        
    </h1>
}
