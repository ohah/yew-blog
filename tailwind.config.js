module.exports = {
	darkMode: "class",
	purge: {
		mode: "all",
		content: ["./src/**/*.rs", "./index.html", "./src/**/*.html", "./src/**/*.css"],
	},
	theme: {},
	variants: {
		scrollbar: ["dark"],
	},
	plugins: [
		require("@tailwindcss/typography"),
		require("@tailwindcss/forms"),
		require("@tailwindcss/line-clamp"),
		require("tailwind-scrollbar"),
	],
};
