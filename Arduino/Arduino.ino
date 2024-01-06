const int DELAY = 75;
const int POTS = 4;
int potValues[] = {0, 0, 0, 0};
int potIds[] = {A0, A1, A2, A3};

void setup()
{
  Serial.begin(115200);
}

void loop()
{
  for (int i = 0; i < POTS; i++) {
    int analogValue = analogRead(potIds[i]);
    // Calculate the value from 0-100 based on the analog value (0-1023)
    int value = map(analogValue, 0, 1023, 0, 100);
    if (value == 99) {
      value = 100;
    }
    potValues[i] = value;
    Serial.print("pot");
    Serial.print(i);
    Serial.print("=");
    Serial.println(value);
  }

  delay(DELAY);
}
