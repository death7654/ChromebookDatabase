# Chromebook Database
<img src="https://img.shields.io/github/downloads/Death7654/ChromebookDatabase/total" alt="shields">&nbsp;&nbsp;
<img src="https://img.shields.io/github/forks/Death7654/ChromebookDatabase?style=social" alt="shields">&nbsp;&nbsp;
<img src="https://img.shields.io/github/stars/Death7654/ChromebookDatabase?style=social" alt="shields">

General database for chromebooks and what drivers they require for windows. Contains ID, boardname, device name, cpu brand, cpu generation, and what drivers are avaliable for the device

To add to the database, simply connect to the database and execute

```
INSERT INTO `chromebooks` (board_name, device_name, cpu_brand, cpu_generation, cpu_codename, avaliable_drivers) VALUES ('board_name','device_name','cpu_brand','cpu_generation','cpu_codename','avaliable_drivers');

```
Export the database, and in terminal run

```
cargo run
```
