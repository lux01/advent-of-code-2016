(ns day-16.core
  (:gen-class))

(defn dragon
  [s]
  (apply str s 0 (map #(condp = % \0 \1, \1 \0) (reverse s))))

(defn fill-to-length
  [n s]
  (apply str
         (take n (first (drop-while #(< (count %) n)
                                    (iterate dragon s))))))

(defn checksum
  ([s]
   (checksum s []))

  ([[c1 c2 & cs] check]
   (if (nil? c2) (apply str check)
       (recur cs (conj check (condp = (list c1 c2)
                               '(\0 \1) \0
                               '(\1 \0) \0
                               '(\0 \0) \1
                               '(\1 \1) \1
                               ))))))

(defn checksum-odd
  [s]
  (first (drop-while (comp even? count)
                     (iterate checksum s))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [input "01000100010010111"]
    (println "Disk 1 checksum:" (checksum-odd (fill-to-length 272 input)))
    (println "Disk 2 checksum:" (checksum-odd (fill-to-length 35651584 input)))))
