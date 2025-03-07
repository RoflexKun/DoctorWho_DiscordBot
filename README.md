# 🤖 Discord Bot for Doctor Who

## 📌 Table of Contents
- [📜 Description](#-description)
- [⚙️ Functionalities](#-functionalities)
  - [🆘 Help Command](#-help-command)
  - [👨‍⚕️ Doctor Pictures](#-doctor-pictures)
  - [📺 Episode Information](#-episode-information)
  - [💬 Quotes](#-quotes)
  - [🎯 Quiz & Points](#-quiz--points)
- [⚠️ Disclaimer](#-disclaimer)

---

## 📜 Description

🚀 This project is a **Discord bot** developed in **Rust** using the ***Serenity*** and ***Tokio*** crates. It also leverages ***JSON*** to store important data such as **episode information**, **leaderboards**, and more.

---

## ⚙️ Functionalities

### 🆘 Help Command

🔹 Use the `+help` command in any chat to display all available bot commands.

<p align="center">
  <img src="https://github.com/user-attachments/assets/096b9cd3-cced-4942-844a-ac42d449dcbc" width="500" />
</p>
<p align="center">🛠️ Fig. 1: Help Command</p>

### 👨‍⚕️ Doctor Pictures 

🔹 Use `+doctor:` followed by a number (1-15) to get an image of the corresponding Doctor. If an invalid number is given, an error message will be displayed.

<p align="center">
  <img src="https://github.com/user-attachments/assets/98fe4ed4-314c-419d-beb1-d4dd22687aa3" width="500" />
</p>
<p align="center">📸 Fig. 2: Correct usage of +doctor command</p>

<p align="center">
  <img src="https://github.com/user-attachments/assets/b43d001d-bc3f-46f8-b44b-cad59c8aadcd" width="500" />
</p>
<p align="center">❌ Fig. 3: Error message for incorrect input</p>

### 📺 Episode Information

🔹 Similar to `+doctor`, the `+episode` command retrieves and displays information about an episode based on the title provided.

<p align="center">
  <img src="https://github.com/user-attachments/assets/1944b4ad-3c3f-4315-8e4e-1d403326064a" width="500" />
</p>
<p align="center">🎬 Fig. 4: Example of an episode query</p>

### 💬 Quotes

🔹 The bot randomly selects a **Doctor Who** quote from a stored text file and displays it when you use `+quote`.

<p align="center">
  <img src="https://github.com/user-attachments/assets/e215acb1-f898-4554-9739-c92925d3da6b" width="500" />
</p>
<p align="center">🗣️ Fig. 5: Example of a randomly selected quote</p>

### 🎯 Quiz & Points

🔹 The bot posts a **quiz question every hour** (unless the previous one is unanswered). If you answer correctly, you earn **1 point**. 

🔹 Use `+points` to check the leaderboard.

<p align="center">
  <img src="https://github.com/user-attachments/assets/4697fc7b-9c2c-4723-add3-1d054f7bb59f" width="500" />
</p>
<p align="center">✅ Fig. 6: Correctly answering a question</p>

<p align="center">
  <img src="https://github.com/user-attachments/assets/1f11937e-9745-4084-a267-cbe8034715c5" width="500" />
</p>
<p align="center">🏆 Fig. 7: Leaderboard display</p>

---

## ⚠️ Disclaimer 

⚠️ **Important Notes:**
- **🔐 The Discord bot token is private** and cannot be shared.
- **📂 File paths are hardcoded** for local use; modifications may be necessary.

📝 *This project was developed for the Rust optional course at UAIC Faculty of Computer Science.*
