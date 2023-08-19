import { type Component, createMemo, Index } from 'solid-js';
import { startFromFile, rawData } from "./audioSource";
import { arc, interpolateSinebow, interpolateInferno } from "d3";

const arcBuilder = arc();

const RadialGraph: Component<{
  color: (value:number) => string;
  scale: number;
}> = (props) => {
  const color = props.color;
  const scale = props.scale;

  const computed = createMemo(() => {
    const data = rawData();

    const total = data.reduce((a,v) => a + v, 0);

    const highCount = data.filter( d => d > 32).length;
    const intensity = highCount / data.length;

    const paths: {
      path: string,
      color: string,
    }[] = [];

    const range = 1.0 + intensity;
    const rangeRadians = range * Math.PI;
    const startAngle = -(rangeRadians / 2);
    // const sliceWidth = rangeRadians / data.length;
    let currentAngle = startAngle;

    for(const d of data) {
      const angle = rangeRadians * (d / total);
      const path = arcBuilder({
        innerRadius: 50 - ((d + 10)/255) * 35,
        outerRadius: 50 + ((d + 10)/255) * 35,
        startAngle: currentAngle,
        endAngle: currentAngle + angle,
      })!;

      paths.push({
        path, color: color(d / 255)
      });

      currentAngle += angle;
    }

    return { paths, intensity };
  });

  return (
    <g transform={`scale(${computed().intensity * scale + 1})`}>
      <Index each={computed().paths}>
        {(p) => <path d={p().path} fill={p().color} />}
      </Index>
    </g>
  );
}

const App: Component = () => {
  return <div onClick={startFromFile}
    style="width: 100vw; height: 100vh;">
    <svg 
      width="100%" 
      height="100%"
      viewBox="-100 -100 200 200"
      preserveAspectRatio="xMidYMid meet"
    >
      <RadialGraph color={interpolateSinebow} scale={2} />
      <RadialGraph color={interpolateInferno} scale={1} />
      <RadialGraph color={interpolateSinebow} scale={0.5} />
    </svg>    
  </div>;
};

export default App;
