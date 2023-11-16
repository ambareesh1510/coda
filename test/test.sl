(def a (C#4:1.5))

(def b (C#4:1.5 D#4:1.5 F#4:1.5 Cbb4:1.5))

(def a (+ 1 2))

(def func (a b) (+ a b))

(out ((f a) (f b))
     f a)

