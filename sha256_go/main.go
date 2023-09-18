package main

import (
	"bytes"
	"crypto/sha256"
	"fmt"
	"io"
	"os"
	"time"
)

var (
	_chars = []byte("0123456789abcdef")
	_names = [][]byte{
		[]byte("zero"), []byte("one"), []byte("two"), []byte("three"),
		[]byte("four"), []byte("five"), []byte("six"), []byte("seven"),
		[]byte("eight"), []byte("nine"), []byte("a"), []byte("b"),
		[]byte("c"), []byte("d"), []byte("e"), []byte("f"),
	}
	_size = len(_chars)
)

func main() {
	start := time.Now()
	self_describe_hash(os.Stdout) // 30.9 secs
	fmt.Println("Go time elapsed is:", time.Since(start).Round(100*time.Millisecond))
}

func self_describe_hash(w io.Writer) {
	var buffer bytes.Buffer
	buffer.WriteString("The SHA256 for this sentence begins with: ")
	baseLen := buffer.Len()

	// 7 for-loops
	for i1 := 0; i1 < _size; i1++ {
		for i2 := 0; i2 < _size; i2++ {
			for i3 := 0; i3 < _size; i3++ {
				for i4 := 0; i4 < _size; i4++ {
					for i5 := 0; i5 < _size; i5++ {
						for i6 := 0; i6 < _size; i6++ {
							for i7 := 0; i7 < _size; i7++ {

								// build sentence
								buffer.Truncate(baseLen)
								buffer.Write(_names[i1])
								buffer.WriteString(", ")
								buffer.Write(_names[i2])
								buffer.WriteString(", ")
								buffer.Write(_names[i3])
								buffer.WriteString(", ")
								buffer.Write(_names[i4])
								buffer.WriteString(", ")
								buffer.Write(_names[i5])
								buffer.WriteString(", ")
								buffer.Write(_names[i6])
								buffer.WriteString(" and ")
								buffer.Write(_names[i7])
								buffer.WriteString(".")

								// compute checksum
								sum := sha256.Sum256(buffer.Bytes())

								// compare checksum
								if byte(i1<<4|i2&0x0f) == sum[0] &&
									byte(i3<<4|i4&0x0f) == sum[1] &&
									byte(i5<<4|i6&0x0f) == sum[2] &&
									byte(i7) == sum[3]>>4 {
									fmt.Fprintln(w, buffer.String())
									fmt.Fprintf(w, "%02x\n", sum)
								}
							}
						}
					}
				}
			}
		}
	}
}
