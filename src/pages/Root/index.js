import React, { useEffect, useRef } from "react";
import Worker from "./renderer.worker.js";

export default function RootPage() {
  const canvasRef = useRef();
  useEffect(() => {
    (async () => {
      const worker = new Worker();
      const offscreen = canvasRef.current.transferControlToOffscreen();
      worker.postMessage({ canvas: offscreen }, [offscreen]);
    })();
  });
  return (
    <div>
      <h1>hello</h1>
      <div>
        <canvas ref={canvasRef} />
      </div>
    </div>
  );
}
