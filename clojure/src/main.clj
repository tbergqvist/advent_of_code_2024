(ns main (:require
          [day1]
          [day2]
          [day3]
          [day4]
          [day5]
          [day6]))

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
  (run-day day1/part1 day1/part2 "1")
  (run-day day2/part1 day2/part2 "2")
  (run-day day3/part1 day3/part2 "3")
  (run-day day4/part1 day4/part2 "4")
  (run-day day5/part1 day5/part2 "5")
  (run-day day6/part1 day6/part2 "6")
  )