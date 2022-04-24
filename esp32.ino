#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#define SSID_NAME "MeeCI"
#define SSID_PASS "12345678"
void setup()
{
  Serial.begin(115200);
  WiFi.mode(WIFI_STA);
  WiFi.disconnect();
  delay(100);
  Serial.println();
  Serial.print("Connecting to ");
  Serial.println(SSID_NAME);
  WiFi.begin(SSID_NAME, SSID_PASS);
  while (WiFi.status() != WL_CONNECTED)
  {
    delay(500);
    Serial.print(".");
  }
  Serial.println("");
  Serial.println("WiFi connected");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());
}
void loop()
{
  if (WiFi.status() == WL_CONNECTED) 
  { 
    bool firstSendData = false;
    String device[] = {"Device#1","Device#2","Device#3"};
    String rnd = device[random(0,3)];
    float data = random(30,50);
    DynamicJsonDocument doc(1024);
    doc["device"] = rnd;
    doc["value"] =  data;
    String json;
    serializeJson(doc, json);
    
    Serial.println(json);
    HTTPClient http; 
    http.begin("http://172.20.10.4:8081/datas"); 
    http.addHeader("Content-Type", "application/json"); 
    int httpCode = http.POST(json); 
    String payload = http.getString();
    
    Serial.print(httpCode); //Print HTTP return code
    //Serial.println(payload); //Print request response payload
    if (payload == "1"){
      Serial.println(" OK");
    }else{
      Serial.println(" Bad Request");
    }
    http.end(); //Close connection
 
    if(httpCode == 200)
    {
          firstSendData = true;
    }
    else if(httpCode != 200 && firstSendData)
    {
          firstSendData = false;
          ESP.restart();  
    }
 }
 delay(10000); //Send a request every 10 seconds
}
