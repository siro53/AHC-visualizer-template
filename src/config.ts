import React from "react"

type Config = {
    usage: {
        problemLink: string,
        isDefaultOpen: boolean
    },
    input: {
        seed: {
            min?: string,
            max?: string
        },
        cases: {
            min?: string,
            max?: string
        },
        rows: number,
        textAreaStyle?: React.CSSProperties
    },
    output: {
        rows: number,
        textAreaStyle?: React.CSSProperties
    }
};

const config : Config = {
    usage: {
        problemLink: "https://atcoder.jp/contests/masters2025-qual",
        isDefaultOpen: true,
    },
    input: {
        seed: {
            min: "0",
            max: "18446744073709551615"
        },
        cases: {
            min: "1",
            max: "10000"
        },
        rows: 4,
        textAreaStyle: {
            width: 650,
            overflowY: "scroll"
        }
    },
    output: {
        rows: 4,
        textAreaStyle: {
            width: 650,
            overflowY: "scroll"
        }
    }
};

export default config;