#!/usr/bin/env python3

import sqlite3

connection = sqlite3.connect("screenings.db")

cursor = connection.cursor()
cursor.execute("DROP TABLE IF EXISTS movies")
cursor.execute("CREATE TABLE IF NOT EXISTS movies (id INTEGER PRIMARY KEY, title TEXT, theater TEXT, screening TEXT)")

connection.commit()
connection.close()
