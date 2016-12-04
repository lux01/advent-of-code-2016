(ns day-04.core
  (:gen-class))

(defn parse-room
  [room-str]
  (let [matches (re-matches #"([a-z\-]+)-([\d]+)\[([a-z]{5})\]" room-str)]
    {:enc-name (nth matches 1)
     :sector (Integer/parseUnsignedInt (nth matches 2))
     :checksum (nth matches 3)}))

(defn verify-checksum
  [room]
  (let [room-name (:enc-name room)]
    (->> (filter (partial not= \-) room-name)
         (frequencies)
         (map vec)
         (sort #(if (= (second %1) (second %2))
                  (compare (first %1) (first %2))
                  (> (second %1) (second %2))))
         (take 5)
         (map first)
         (= (seq (:checksum room))))))

(defn decode-character
  [rot c]
  (if (= \- c) \space
      (char (+ (int \a)
               (mod (+ (- (int c) (int \a)) rot)
                    26)))))

(defn decrypt-room
  [room]
  (assoc room :name (apply str (map (partial decode-character (:sector room)) (:enc-name room)))))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (->> (slurp "input.txt")
       (clojure.string/split-lines)
       (map parse-room)
       (filter verify-checksum)
       (map :sector)
       (reduce +)
       (println))

  (->> (slurp "input.txt")
       (clojure.string/split-lines)
       (map parse-room)
       (filter verify-checksum)
       (map decrypt-room)
       (filter #(clojure.string/includes? % "object"))
       (println)))
