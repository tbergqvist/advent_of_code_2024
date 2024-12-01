(ns main (:require [day1]))

(defmacro time2 [expr]
  `(let [start# (System/nanoTime)
        result# ~expr
        end# (System/nanoTime)]
    (str result# ", time: " (/ (- end# start#) 1000000.0) "ms")
    ))

(defn run-day [fn1 fn2 day]
  (let [input (slurp (str "../inputs/" day ".txt"))]
    (println (time2 (str day "a: " (fn1 input))))
    (println (time2 (str day "b: " (fn2 input))))))

(defn -main [& _]
  (run-day day1/part1 day1/part2 "1"))