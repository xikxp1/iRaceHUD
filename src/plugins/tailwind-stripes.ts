import plugin from "tailwindcss/plugin"

export default plugin.withOptions(
    function () {
        return function ({ e, addBase, addComponents, addUtilities, matchUtilities, theme }: any) {
            addBase({
                "@keyframes slides": {
                    from: { transform: "translateX(0)" },
                    to: { transform: "translateX(var(--stripes-size))" },
                },
                ":root": {
                    "--stripes-color": "0 0 0",
                    "--stripes-angle": "-45deg",
                    "--stripes-speed": "1s",
                    "--stripes-direction": "forwards",
                    "--stripes-opacity": "1",
                    "--stripes-size": "20px",
                    "--stripes-width": "10%",
                },
            })

            addComponents({
                ".stripes": {
                    position: "relative",
                    overflow: "hidden",
                    isolation: "isolate",
                },
                ".stripes:before": {
                    content: '""',
                    position: "absolute",
                    top: "0",
                    right: "0",
                    width: "calc(100% + var(--stripes-size))",
                    height: "100%",
                    pointerEvents: "none",
                    backgroundImage: `linear-gradient(var(--stripes-angle), var(--stripes-color) 5%, transparent 5%, transparent 45%, var(--stripes-color) 45%, var(--stripes-color) 55%, transparent 55%, transparent 95%, var(--stripes-color) 95%)`,
                    backgroundSize: "var(--stripes-size) var(--stripes-size)",
                    animation: "var(--stripes-speed) slides infinite linear var(--stripes-direction)",
                },
            })

            addUtilities({
                ".stripes-reverse": { "--stripes-direction": "reverse" },
            })

            matchUtilities(
                {
                    "stripes-opacity": (value: any) => ({
                        "--stripes-opacity": value,
                    }),
                },
                {
                    values: theme("opacity"),
                }
            )

            matchUtilities(
                {
                    "stripes-size": (value: any) => ({
                        "--stripes-size": value,
                    }),
                },
                {
                    values: theme("stripeSizes"),
                }
            )

            matchUtilities(
                {
                    "stripes": (value: any) => {
                        return {
                            "--stripes-color": value.replace("<alpha-value>", "var(--stripes-opacity)"),
                        }
                    },
                },
                {
                    values: theme("colors"),
                }
            )
        }
    },
    function () {
        return {
            theme: {
                stripeSizes: {
                    "xs": "6px",
                    "sm": "12px",
                    "md": "20px",
                    "lg": "30px",
                    "xl": "40px",
                    "2xl": "50px",
                    "3xl": "60px",
                },
            },
        }
    }
)
