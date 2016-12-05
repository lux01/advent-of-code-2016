(defproject day_05 "1.0.0"
  :description "Clojure solution to day 5 of the Advent of Code 2016"
  :url "https://github.com/lux01/advent-of-code-2016"
  :license {:name "MIT License"
            :url "https://github.com/lux01/advent-of-code-2016/blob/master/LICENSE"}
  :dependencies [[org.clojure/clojure "1.8.0"]
                 [digest "1.4.5"]]
  :main ^:skip-aot day-05.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
