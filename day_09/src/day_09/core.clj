(ns day-09.core
  (:gen-class))

(defn decompressed-len
  ([f s]
   (decompressed-len f s 0))

  ([f s len]
      (if (empty? s) len
         (let [[unused comp-len] (f s)]
           (recur f unused (+ len comp-len))))))

(defn process-char-v1
  [s]
  (if (=\( (first s))
    (let [[_ num-take num-reps remaining] (re-matches #"\((\d+)x(\d+)\)(.*)"
                                                      (apply str s))
          [taken unused] (split-at (Integer. num-take) remaining)]
      (list unused (* (Integer. num-reps) (count taken))))
    (list (rest s) 1)))

(defn process-char-v2
  [s]
  (if (=\( (first s))
    (let [[_ num-take num-reps remaining] (re-matches #"\((\d+)x(\d+)\)(.*)"
                                                      (apply str s))
          [taken unused] (split-at (Integer. num-take) remaining)]
      (list unused                              
            (* (Integer. num-reps) (decompressed-len process-char-v2 taken 0))))
    (list (rest s) 1)))


(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [input-file (filter #(not (Character/isWhitespace %)) (slurp "input.txt"))]
    (println "v1 Length = " (decompressed-len process-char-v1 input-file))
    (println "v2 Length = " (decompressed-len process-char-v2 input-file))))
