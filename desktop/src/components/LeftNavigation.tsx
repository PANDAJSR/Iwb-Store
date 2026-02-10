import React from 'react';
import { Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Toolbar } from '@mui/material';
import AppsIcon from '@mui/icons-material/Apps';

interface LeftNavigationProps {
  selectedItem: string;
  onSelectItem: (itemId: string) => void;
}

const LeftNavigation: React.FC<LeftNavigationProps> = ({ selectedItem, onSelectItem }) => {
  const drawerWidth = 120;

  return (
    <Drawer
      variant="permanent"
      sx={{
        width: drawerWidth,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: drawerWidth,
          boxSizing: 'border-box',
          borderRight: '1px solid',
          borderColor: 'divider',
        },
      }}
    >
      <Toolbar />
      <List sx={{ pt: 0 }}>
        <ListItem disablePadding>
          <ListItemButton
            selected={selectedItem === 'appLibrary'}
            onClick={() => onSelectItem('appLibrary')}
            sx={{
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
              minHeight: '80px',
              py: 2,
              px: 1,
              transition: 'all 0.2s',
            }}
          >
            <ListItemIcon
              sx={{
                minWidth: 'auto',
                mb: 1,
                color: selectedItem === 'appLibrary' ? 'primary.main' : 'inherit',
              }}
            >
              <AppsIcon sx={{ fontSize: '28px' }} />
            </ListItemIcon>
            <ListItemText
              primary="应用库"
              sx={{
                '& .MuiListItemText-primary': {
                  fontSize: '14px',
                  textAlign: 'center',
                  fontWeight: selectedItem === 'appLibrary' ? 500 : 400,
                }
              }}
            />
          </ListItemButton>
        </ListItem>
      </List>
    </Drawer>
  );
};

export default LeftNavigation;