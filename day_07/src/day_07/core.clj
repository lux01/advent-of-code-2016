(ns day-07.core
  (:gen-class))

(defn has-abba?
  "Checks if the string s has an ABBA."
  [s]
  (not (nil? (re-matches #"(?:\w*?)(\w)((?:(?!\1)\w))\2\1\w*" s))))

(defn find-aba-bab
  "Finds all the ABAs in IP map."
  [ip]
  (let [regex #"(?=(\w)((?:(?!\1)\w))\1)"
        abas (mapcat (partial re-seq regex) (:supernet ip))
        babs (mapcat (partial re-seq regex) (:hypernet ip))]
    {:aba (map rest abas), :bab (map rest babs)}))

(defn ssl?
  "Tests if the given IP map supports SSL."
  [ip]
  (let [parts (find-aba-bab ip)]
    (some identity (for [aba (:aba parts)
                         bab (:bab parts)]
                     (= aba (reverse bab))))))
  
(defn get-babs
  "Finds all the BABs in an IP map."
  [ip]
  (map rest
       (mapcat (partial re-seq #"(?:\w*?)(\w)((?:(?!\1)\w))\1\w*")
               (:hypernet ip))))

(defn split-ip
  "Splits the IP into two sequences of supernet and hypernet sequences."
  [s]
  {:hypernet (map last (re-seq #"\[(\w+)\]" s)),
   :supernet (map last (re-seq #"(?:^|\])(\w+)" s))})

(defn tls?
  "Checks if the given IP supports TLS."
  [ip]
  (and (not-any? has-abba? (:hypernet ip))
       (some has-abba? (:supernet ip))))

(defn -main
  [& args]
  (let [ips (map split-ip (clojure.string/split-lines (slurp "input.txt")))]
    (println "Number of TLS enabled IPs = " (count (filter tls? ips)))
    (println "Numebr of SSL enabled IPs = " (count (filter ssl? ips)))))
