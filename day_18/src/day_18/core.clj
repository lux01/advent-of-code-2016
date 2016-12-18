(ns day-18.core
  (:gen-class))

(def test-input
  (map (partial = \^) ".^^.^.^^^^"))

(def input
  (->> "^^^^......^...^..^....^^^.^^^.^.^^^^^^..^...^^...^^^.^^....^..^^^.^.^^...^.^...^^.^^^.^^^^.^^.^..^.^"
      (map (partial = \^))))

(defn is-trap?
  [left centre right]
  (cond
    (and left centre (not right)) true,
    (and centre right (not left)) true,
    (and left (not centre) (not right)) true,
    (and (not left) (not centre) right) true,
    :else false))

(defn next-line
  [line]
  (->> (range 0 (count line))
       (map #(is-trap? (nth line (dec %) false)
                       (nth line %)
                       (nth line (inc %) false)))))

(defn num-safe
  [num-lines input]
  (count (mapcat (partial filter not)
                 (reduce conj [] (take num-lines (iterate next-line input))))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Number of safe tiles in 40 rows:" (num-safe 40 input))
  (println "Number of safe tiles in 400000 rows:" (num-safe 400000 input)))
