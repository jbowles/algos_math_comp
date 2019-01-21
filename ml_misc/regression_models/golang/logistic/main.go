package main

import (
	"encoding/csv"
	"fmt"
	"log"
	"math"
	"math/rand"
	"os"
	"strconv"
	"time"

	"gonum.org/v1/gonum/mat"
)

func main() {
	//runRandomSample()
	runFICOSample()
	//fmt.Printf("%v\n", sigmoid(7.0))
}

/*
	calculate logit for logistic function:
	f(x) = 1/1+e^x
*/
func sigmoid(e float64) float64 {
	return 1 / (1 + math.Exp(-e))
}

func seedRand() *rand.Rand {
	return rand.New(rand.NewSource(time.Now().UnixNano()))
}

func logisticRegression(features *mat.Dense, labels []float64, numSteps int, learningRate float64) []float64 {
	//initialize random weights
	_, numWeights := features.Dims()
	weights := make([]float64, numWeights)

	//seed rand and populate random weigths
	r := seedRand()
	for idx := range weights {
		weights[idx] = r.Float64()
	}

	//loop optimize the weights
	for i := 0; i < numSteps; i++ {
		//init for error accumulate
		var sumError float64
		//make preditions for each label and accumulate error
		for idx, label := range labels {
			//get features corresponding with label, length of features (100)
			featureRow := mat.Row(nil, idx, features)
			//fmt.Printf("\nfeatureRow length: %v\n", len(featureRow))

			//calculate error for this iteration weights
			var sumWeight float64
			for x, f := range featureRow {
				sumWeight += f * weights[x]
			}
			pred := sigmoid(sumWeight)
			//pred := sigmoid(featureRow[0]*weights[0] + featureRow[1]*weights[1])
			predError := label - pred
			sumError += math.Pow(predError, 2)

			//update feature weigts
			for j := 0; j < len(featureRow); j++ {
				weights[j] += learningRate * predError * pred * (1 - pred) * featureRow[j]
			}
		}
	}

	return weights
}

/*
func runRandomSample() {
	size := 100
	inputData := make([]float64, size*100)
	r := seedRand()
	for i := 0; i < size*100; i++ {
		inputData[i] = r.Float64()
	}
	data := mat.NewDense(size, size, inputData)
	w := logisticRegression(data, []float64{0, 1, 2, 3, 4}, 20, 0.0001)
	for _, val := range w {
		fmt.Printf("%v\n", val)
	}
}
*/

func runFICOSample() {
	// Open the training dataset file.
	f, err := os.Open("training.csv")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// Create a new CSV reader reading from the opened file.
	reader := csv.NewReader(f)
	reader.FieldsPerRecord = 2

	// Read in all of the CSV records
	rawCSVData, err := reader.ReadAll()
	if err != nil {
		log.Fatal(err)
	}

	// featureData and labels will hold all the float values that
	// will eventually be used in our training.
	featureData := make([]float64, 2*(len(rawCSVData)-1))
	labels := make([]float64, len(rawCSVData)-1)

	// featureIndex will track the current index of the features
	// matrix values.
	var featureIndex int

	// Sequentially move the rows into the slices of floats.
	for idx, record := range rawCSVData {

		// Skip the header row.
		if idx == 0 {
			continue
		}

		// Add the FICO score feature.
		featureVal, err := strconv.ParseFloat(record[0], 64)
		if err != nil {
			log.Fatal(err)
		}

		featureData[featureIndex] = featureVal

		// Add an intercept.
		featureData[featureIndex+1] = 1.0

		// Increment our feature row.
		featureIndex += 2

		// Add the class label.
		labelVal, err := strconv.ParseFloat(record[1], 64)
		if err != nil {
			log.Fatal(err)
		}

		labels[idx-1] = labelVal
	}

	// Form a matrix from the features.
	features := mat.NewDense(len(rawCSVData)-1, 2, featureData)

	// Train the logistic regression model.
	weights := logisticRegression(features, labels, 1000, 0.3)

	// Output the Logistic Regression model formula to stdout.
	formula := "p = 1 / ( 1 + exp(- m1 * FICO.score - m2) )"
	fmt.Printf("\n%s\n\nm1 = %0.2f\nm2 = %0.2f\n\n", formula, weights[0], weights[1])
}
