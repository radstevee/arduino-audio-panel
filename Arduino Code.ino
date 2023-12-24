const int POTS = 3;
const int DELAY = 100;


int potPids[] = {A2, A1, A0};
int potVals[] = {0, 0, 0};


void setup() {
  Serial.begin(9600);
  Serial.print("Started\n");
}


void loop() {
  for (int i = 0; i < POTS; i++) {
    potVals[i] = analogRead(potPids[i]);
    float percentage = (float) potVals[i] / 1023 * 100;
    int roundedPercentage = (int) floor(percentage);
    Serial.print("pot");
    Serial.print(i);
    Serial.print("=");
    //Serial.print(roundedPercentage);
    //Serial.println("%");
    Serial.println(potVals[i]);
  }


  delay(DELAY);
}