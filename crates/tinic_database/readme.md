# Tinic Database

**Tinic Database** is a module created to make working with game databases much easier for frontend developers.

RetroArch **RDB files**, while comprehensive, are not very friendly for reading, searching, or integrating into modern applications. **Tinic Database** solves this by leveraging the RDB reading functionality provided by **Tinic Super**. Once the data is supplied by **Tinic Super**, you can use **Tinic Database** to store it in an **SQLite** database and take advantage of SQLite’s powerful querying and data access features.

---

## 🎯 Purpose

Provide a game data access layer that is:

- Easy to integrate
- Fast for queries
- Developer-friendly
- Independent from the original RDB format

---

## 🦀 Current Support

At the moment, the planned usage of Tinic Database is focused on **Rust**, with APIs designed to be easy to use within the Tinic ecosystem.  
If you are using other programming languages, you will need to create your own database using the technology of your choice.

If you are using Flutter and do not want to build a database using libraries from the native Flutter ecosystem, you can use [Rinf](https://rinf.cunarist.org/) to take advantage of Tinic Database.  
If you need an example of how this works in practice, check out [Retronic](https://github.com/Xsimple1010/retronic/tree/master/native).
