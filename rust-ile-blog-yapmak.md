---
title: "Rust ile Kendi Blogunu Yapmak"
date: 2025-11-03
slug: "rust-ile-blog-yapmak"
tags: ["rust", "web", "markdown", "axum"]
description: "Axum ve SQLite kullanarak basit bir markdown tabanlı blog API'si oluşturmayı öğreniyoruz."
---

# Rust ile Blog Yapmak

Rust artık sadece sistem programlama dili değil — **web backend** tarafında da gayet güçlü bir oyuncu.  
Bu yazıda, Markdown formatında içerik alabilen ve REST API olarak çalışan bir blog motoru kuracağız.

## Başlangıç

Projeyi başlatmak için önce bir `Cargo` projesi oluşturuyoruz:

```bash
cargo new my_blog
cd my_blog
```

Ardından `Cargo.toml` içine şu bağımlılıkları ekliyoruz:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
markdown = "1"
chrono = "0.4"
```

## Blog Modeli

Bir blog yazısı için gereken temel alanlar genellikle şunlardır:

```rust
#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
    content_markdown: String,
    created_at: DateTime<Utc>,
    slug: String,
    tags: Vec<String>,
}
```

Bu modeli SQLite veritabanında saklayabiliriz.

## Markdown Desteği

Rust’ta Markdown’ı HTML’ye çevirmek için `markdown-rs` veya `pulldown-cmark` kullanılabilir.  
Basit örnek:

```rust
let md = "# Merhaba Dünya!";
let html = markdown::to_html(md);
println!("{}", html);
```

> Çıktı: `<h1>Merhaba Dünya!</h1>`

## Sonuç

Artık hem **Markdown formatında içerik saklayan**,  
hem de **Axum tabanlı bir REST API** üzerinden bu içerikleri sunabilen bir sistem kurduk.

Bundan sonra, bu API’ye bağlı bir frontend veya static site generator ekleyebiliriz.

---

**Kaynaklar**
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [markdown-rs](https://crates.io/crates/markdown)
- [Rust Book](https://doc.rust-lang.org/book/)
