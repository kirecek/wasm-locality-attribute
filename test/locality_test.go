package locality

import (
	"testing"
	"time"

	"istio.io/proxy/test/envoye2e/driver"
	"istio.io/proxy/testdata"

	"github.com/istio-ecosystem/wasm-extensions/test"
)

func TestLocalityAttribute(t *testing.T) {
	params := driver.NewTestParams(t, map[string]string{
		"ExampleWasmFile": "example.wasm",
	}, test.ExtensionE2ETests)
	params.Vars["ServerHTTPFilters"] = params.LoadTestData("test/testdata/server_filter.yaml.tmpl")
	params.Vars["ClientHTTPFilters"] = params.LoadTestData("test/testdata/client_filter.yaml.tmpl")
	if err := (&driver.Scenario{
		Steps: []driver.Step{
			&driver.XDS{},
			&driver.Update{
				Node: "server", Version: "0", Listeners: []string{string(testdata.MustAsset("listener/server.yaml.tmpl"))},
			},
			&driver.Envoy{
				Bootstrap:       params.FillTestData(params.LoadTestData("test/testdata/bootstrap.yaml.tmpl")),
				DownloadVersion: "1.11",
			},
			&driver.Sleep{Duration: 1 * time.Second},
			&driver.HTTPCall{
				Port:            params.Ports.ServerPort,
				Method:          "GET",
				ResponseHeaders: map[string]string{"x-envoy-peer-zone": driver.Any},
				RequestHeaders:  map[string]string{"x-envoy-peer-zone": driver.Any},
			},
		},
	}).Run(params); err != nil {
		t.Fatal(err)
	}
}
