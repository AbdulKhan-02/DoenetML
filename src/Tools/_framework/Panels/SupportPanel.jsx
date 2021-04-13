import React from 'react';
import styled from 'styled-components';

const SupportWapper = styled.div`
  overflow: auto;
  grid-area: supportPanel;
  background-color: hsl(0, 0%, 99%);
  height: 100%;
  border-radius: 0 0 4px 4px;
`;

const ControlsWrapper = styled.div`
  grid-area: supportControls;
  display: flex;
  gap: 4px;
  background-color: hsl(0, 0%, 89%);
  border-radius: 4px 4px 0 0;
`;

export default function SupportPanel({ children, responsiveControls }) {
  return (
    <>
      {responsiveControls ? (
        <ControlsWrapper>{responsiveControls}</ControlsWrapper>
      ) : null}
      <SupportWapper>{children}</SupportWapper>
    </>
  );
}
