# Tutorial 8 - Understanding Subscriber and Message Broker

## Jawaban Pertanyaan

### a. Apa itu AMQP?
**AMQP (Advanced Message Queuing Protocol)** adalah protokol standar terbuka di lapisan aplikasi yang digunakan untuk komunikasi antar sistem melalui pengiriman pesan (messaging). AMQP memungkinkan aplikasi untuk saling terhubung melalui *Message Broker* (seperti RabbitMQ) meskipun aplikasi tersebut dibangun dengan bahasa pemrograman yang berbeda atau berjalan di platform yang berbeda. AMQP menjamin efisiensi, keamanan, dan keandalan dalam pengiriman pesan (asynchronous messaging).

### b. Apa arti dari `guest:guest@localhost:5672`?
Format string tersebut adalah sebuah URL koneksi untuk mengakses *Message Broker*. Berikut adalah rinciannya:

* **guest (pertama):** Merupakan **Username** default yang digunakan untuk autentikasi.
* **guest (kedua):** Merupakan **Password** default yang digunakan untuk autentikasi.
* **localhost:** Merupakan alamat **Hostname** atau lokasi server tempat message broker berjalan. `localhost` berarti server berada di komputer yang sama dengan aplikasi yang sedang dijalankan.
* **5672:** Merupakan **Port** default yang digunakan oleh RabbitMQ untuk mendengarkan koneksi yang masuk menggunakan protokol AMQP.

---