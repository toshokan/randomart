# randomart

Generate OpenSSH randomart from fingerprints.

[![GitHub top language](https://img.shields.io/github/languages/top/toshokan/indirect-once)](https://github.com/toshokan/randomart)

## Usage

We can view the SHA256 and MD5 randomart for a key using `ssh-keygen`.

For a test key I generated, the output of `ssh-keygen -lvf test.pub` is:
```
3072 SHA256:l2Gb0n5mi0b6T8esAK3OX7OixVjKO9KsTBpvRwddfK4 toshokan@shojigate.net (RSA)
+---[RSA 3072]----+
|           .     |
|            o .  |
|         .o. o   |
|        .+.=  .  |
|        S.O  .   |
|       ..@o Eo   |
|    . .+=+= B +  |
|     *oo*ooO *   |
|    ..++=*+o=    |
+----[SHA256]-----+
```
The output of `echo "l2Gb0n5mi0b6T8esAK3OX7OixVjKO9KsTBpvRwddfK4" | randomart --base64` is:
```
+-----------------+
|           .     |
|            o .  |
|         .o. o   |
|        .+.=  .  |
|        S.O  .   |
|       ..@o Eo   |
|    . .+=+= B +  |
|     *oo*ooO *   |
|    ..++=*+o=    |
+-----------------+
```
Likewise, for an MD5 fingerprint, the output of `ssh-keygen -E md5 -lvf test.pub` is:
```
3072 MD5:ea:56:0e:02:3b:41:ec:f8:3a:53:23:ee:bb:33:a5:57 toshokan@shojigate.net (RSA)
+---[RSA 3072]----+
| .               |
|  o              |
| +               |
|. +              |
| . +    S        |
|. *..E...        |
|.+oo...+         |
|+= . .. .        |
|.**  ..          |
+------[MD5]------+
```
The output of `echo "ea560e023b41ecf83a5323eebb33a557" | randomart` is:
```
+-----------------+
| .               |
|  o              |
| +               |
|. +              |
| . +    S        |
|. *..E...        |
|.+oo...+         |
|+= . .. .        |
|.**  ..          |
+-----------------+
```

Note that the headers and footers do not contain the same information, since `randomart` only sees the hash (and knows nothing about the key that hashed to it).

## License
The algorithms used in `randomart` are adapted/derived from (and intended to be nearly identical to) the randomart algorithms in OpenSSH. 

`randomart` is licensed under the 3-clause BSD license, similar to OpenSSH.
