(ns day-06.core
  (:gen-class))

(def test-input
  "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar")

(defn transpose
  [ls]
  (apply mapv vector ls))

(defn decode-message
  ([msg selector]
   (->> (clojure.string/split-lines msg)
        (map seq)
        (transpose)
        (map (comp first #(apply selector second %) frequencies))
        (apply str)))
  ([msg]
   (decode-message msg max-key)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println (decode-message (slurp "input.txt")))
  (println (decode-message (slurp "input.txt")
                           min-key)))
