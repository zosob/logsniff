import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
from sklearn.cluster import DBSCAN
import sys

df = pd.read_csv(sys.argv[1])

from sklearn.preprocessing import StandardScaler
X = df[['line_length', 'token_count']]
X_scaled = StandardScaler().fit_transform(X)

db = DBSCAN(eps=0.5, min_samples=2).fit(X_scaled)
df['anomaly'] = db.labels_

df.to_csv("analyzed.csv", index = False)

sns.scatterplot(data=df, x='line_length', y ='token_count', hue='anamoly', palette='deep')
plt.title("Anonmaly Detection via DBSCAN")
plt.savefig("anomalies.png")
plt.show()