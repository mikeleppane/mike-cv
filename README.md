# Mike CV - Professional Resume in Rust 🦀

A modern, responsive CV/resume website built with Rust, Leptos, and Tailwind CSS. This project showcases professional experience, skills, and achievements in an elegant web format.

## 🚀 Features

- **Modern Design**: Clean, professional layout with gradient backgrounds and hover effects
- **Responsive**: Optimized for desktop, tablet, and mobile devices
- **Interactive Components**: Dynamic sections with smooth animations
- **Fast Performance**: Built with Rust and WebAssembly for optimal speed
- **SEO Friendly**: Proper meta tags and semantic HTML structure

## 🛠️ Tech Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) - Rust web framework
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- **Language**: Rust 🦀
- **Build Tool**: Cargo Leptos
- **Deployment**: WebAssembly (WASM)

## 📋 Sections

- **Professional Summary**: Overview of expertise and career highlights
- **Experience**: Detailed work history with achievements
- **Education**: Academic background and qualifications
- **Skills**: Technical skills organized by categories
- **Languages**: Language proficiencies with visual indicators
- **Certifications**: Professional certifications and credentials
- **Continuous Learning**: Ongoing education and development
- **GitHub**: Repository showcase and contribution activity

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Cargo Leptos](https://github.com/leptos-rs)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/mike-cv-in-rust.git
cd mike-cv-in-rust
```

2. Install Cargo Leptos:
```bash
cargo install cargo-leptos
```

3. Run the development server:
```bash
cargo leptos watch
```

4. Open your browser and navigate to `http://localhost:3000`

### Building for Production

```bash
cargo leptos build --release
```

## 📁 Project Structure

```
src/
├── app.rs              # Main application component
├── components/         # Reusable UI components
│   ├── personal_info.rs
│   ├── summary.rs
│   ├── experience.rs
│   ├── education.rs
│   ├── skills.rs
│   ├── languages.rs
│   ├── certifications.rs
│   ├── continuous_learning.rs
│   └── github.rs
├── lib.rs              # Library root
└── main.rs             # Application entry point
```

## 🎨 Customization

The CV content can be easily customized by modifying the component data structures in each respective component file. Colors and styling can be adjusted through Tailwind CSS classes.

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/yourusername/mike-cv-in-rust/issues).

## 👤 Author

**Mikko Leppänen**
- LinkedIn: [Mikko Leppänen](https://www.linkedin.com/in/mikko-lepp%C3%A4nen-05bb621a/)
- Blog: [Medium](https://medium.com/@mleppan23)
- Email: mleppan23@gmail.com

---

Built with ❤️ and Rust 🦀