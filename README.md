# Rust exercises

This package implements 3 different rust exercies. Run `cargo test` to run the corresponding test for the first two exercises.
The 3rd task can be run with `cargo run`

## On 3rd task
I was unable to get this to work within a reasonable time frame and have decided to not implement it fully.

I understand that I should take the binary array and remove `13u8` from each value and the get the encoded string.
However using the standard `write` method I am unable to manipulate the actual bytes. As per the documentation

(The write method will attempt to write some data into the object, returning how many bytes were successfully written.)[https://doc.rust-lang.org/std/io/trait.Write.html]

I understand that I should have used the `BufWriter` in some capacity since it looks to allow for byte manipulation. According to the docs it looked like I would be able to wrap my `self.content` inside a `BufWriter` since it implemented the `Write` method but I had no such sucess.

```
let mut buffer = BufWriter::new(self.content);
   |                                         ^^^^^^^^^^^^ move occurs because `self.content` has type `T`, which does not implement the `Copy` trait
```

After some testing and looking online I have decided to not continue with the final task of decoding the message. I was able to copy the bytes from one array to another, that is how far I got.