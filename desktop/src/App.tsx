import { useState } from 'react';
import { Box, Container } from '@mui/material';
import LeftNavigation from './components/LeftNavigation';
import AppLibrary from './components/AppLibrary';

function App() {
  const [selectedItem, setSelectedItem] = useState('appLibrary');

  return (
    <Box sx={{ display: 'flex', height: '100vh' }}>
      <LeftNavigation
        selectedItem={selectedItem}
        onSelectItem={setSelectedItem}
      />
      <Container maxWidth={false} sx={{ flex: 1, pt: 3 }}>
        {selectedItem === 'appLibrary' && <AppLibrary />}
      </Container>
    </Box>
  );
}

export default App
