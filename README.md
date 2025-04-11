# rusty-svg

<p align="center">
  <img src="https://github.com/user-attachments/assets/27a5badc-8d8a-4d36-be83-19bce1c22cf1" width="300" height="300" />
</p>

Convert SVG files into React components (TypeScript or JavaScript) with ease.  
Supports interactive CLI and configuration via `rusty-svg.config.toml`.
---

## ✨ Features

- ✅ Convert `.svg` files to `.tsx` or `.jsx` React components
- ✅ Automatically applies props: `<svg {...props}>`
- ✅ Interactive CLI prompts or command-line options
- ✅ Custom component prefix (e.g. `IconHome`)
- ✅ Optional config file (`rusty-svg.config.toml`)

---

## 🚀 Installation

```bash
npm i -D rusty-react-svg
```

## 🧪 Usage

### CLI options

```bash
rusty-svg --input icons --output components --typescript
```

### Interactive mode (no arguments)

```bash
rusty-svg
```

You will be prompted to choose:

- Input folder
- Output folder
- Whether to use TypeScript
- Overwrite confirmation if output folder exists

## ⚙️ Config File (rusty-svg.config.toml)

If present, rusty-svg will use this file automatically.

```toml
input = "icons"
output = "components"
typescript = true
prefix = "Icon"
```

Config file overrides prompts unless explicitly overridden via CLI flags.

## 💡 Example Output

If home.svg exists, the result will be:

### TypeScript (tsx)

```tsx
import React from 'react';

type Props = React.SVGProps<SVGSVGElement>;

const IconHome = (props: Props) => <svg {...props}>...</svg>;

export default IconHome;
```

### JavaScript (jsx)

```jsx
import React from 'react';

const IconHome = (props) => <svg {...props}>...</svg>;

export default IconHome;
```

## 📦 In a React Project

Add a script in your package.json:

```json
"scripts": {
  "generate:icons": "rusty-svg"
}
```

Then run:

```bash
npm run generate:icons
```

Ensure `~/.cargo/bin` is in your PATH if you installed rusty-svg with cargo install.

## 🛠 Roadmap

- [ ] index.ts generator
- [ ] Option to optimize SVG (like SVGO)
- [ ] Custom config file path (--config ./my-config.toml)
- [ ] --no-config flag to ignore config file
