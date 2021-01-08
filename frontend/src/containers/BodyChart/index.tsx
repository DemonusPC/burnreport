import React from 'react';
import { VictoryChart, VictoryLine, VictoryScatter, VictoryTooltip } from 'victory';

interface ChartData<X, Y> {
    x: X;
    y: Y;
}

interface BodyChartProps<X, Y> {
    data: Array<ChartData<X,Y>>,
    parseDomain: (data: Array<ChartData<X,Y>>) => [number, number] 
}

const BodyChart = <X,Y,>({data, parseDomain}: BodyChartProps<X,Y>) => {
    return (        <VictoryChart>
        <VictoryLine
          style={{
              labels: {
                  fontSize: 10
              }
          }}
          domain={{y: parseDomain(data)}}
          interpolation="monotoneX"
          data={data}
        />
        
      <VictoryScatter data={data}
          size={3}
          style={{ data: { fill: "#c43a31", stroke: 0.1, strokeWidth: 1 } }}
          labels={({ datum }) => `${datum.y} %`}
          labelComponent={<VictoryTooltip  />}
        />
      </VictoryChart>)
} 

export default BodyChart;