window.onload = function() {
  //<editor-fold desc="Changeable Configuration Block">

  // the following lines will be replaced by docker/configurator, when it runs in a docker-container
  window.ui = SwaggerUIBundle({
    urls: [
      {
        name: "Market API",
        url: "specs/market-api.yaml",
      },
      {
        name: "Activity API",
        url: "specs/activity-api.yaml",
      },
      {
        name: "Payment API",
        url: "specs/payment-api.yaml",
      },
      {
        name: "NET API",
        url: "specs/net-api.yaml",
      },
      {
        name: "NET API v2",
        url: "specs/net-api-v2.yaml",
      },
      {
        name: "GSB API",
        url: "specs/gsb-api.yaml",
      },
    ],
    dom_id: '#swagger-ui',
    deepLinking: true,
    presets: [
      SwaggerUIBundle.presets.apis,
      SwaggerUIStandalonePreset
    ],
    plugins: [
      SwaggerUIBundle.plugins.DownloadUrl
    ],
    layout: "StandaloneLayout"
  });

  //</editor-fold>
};
