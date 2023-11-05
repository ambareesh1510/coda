(let a (C#4:1.5))
(let b (C#4:1.5 D#4:1.5 F#4:1.5 Cbb4:1.5))

(fn f (freq)
  (+ (sint freq) (cost freq))
)

(out ((f a) (f b))
     f a)
