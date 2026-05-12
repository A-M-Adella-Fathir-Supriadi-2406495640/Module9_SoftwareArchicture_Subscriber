# Tutorial 8 - Understanding Subscriber and Message Broker

## Link commit gambar
https://drive.google.com/drive/folders/1beGB2Wqq6JWSt3mrYyfPMhI2buMBMf8B?usp=sharing

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

### how the spike got to do with running the publisher.
Munculnya lonjakan atau spike pada grafik dashboard RabbitMQ terjadi karena adanya aktivitas pengiriman pesan yang masuk ke sistem secara mendadak saat program publisher dijalankan. Ketika kamu mengeksekusi kode tersebut, publisher akan mengirimkan lima buah pesan sekaligus dalam waktu yang sangat singkat, sehingga grafik Message Rate akan langsung naik secara vertikal untuk mencerminkan volume data yang diterima oleh broker.

Jika program subscriber tidak sedang aktif, pesan-pesan ini akan ditampung terlebih dahulu di dalam antrean, yang kemudian menyebabkan grafik Queued Messages menunjukkan angka yang lebih tinggi. Lonjakan ini pada dasarnya adalah representasi visual dari beban kerja sesaat yang diproses oleh RabbitMQ dalam menerima, memvalidasi, dan menyimpan pesan-pesan tersebut di memori sebelum nantinya diambil oleh subscriber.
